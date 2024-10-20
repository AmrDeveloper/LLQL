use std::path::Path;

use arguments::Arguments;
use arguments::Command;
use atty::Stream;
use functions::llvm_ir_function_signatures;
use functions::llvm_ir_functions;
use gitql_cli::arguments::OutputFormat;
use gitql_cli::diagnostic_reporter;
use gitql_cli::diagnostic_reporter::DiagnosticReporter;
use gitql_cli::printer::base::OutputPrinter;
use gitql_cli::printer::csv_printer::CSVPrinter;
use gitql_cli::printer::json_printer::JSONPrinter;
use gitql_cli::printer::table_printer::TablePrinter;
use gitql_core::environment::Environment;
use gitql_core::schema::Schema;
use gitql_engine::data_provider::DataProvider;
use gitql_engine::engine;
use gitql_engine::engine::EvaluationResult::SelectedGroups;
use gitql_parser::diagnostic::Diagnostic;
use gitql_parser::parser;
use gitql_parser::tokenizer;
use gitql_std::aggregation::aggregation_function_signatures;
use gitql_std::aggregation::aggregation_functions;
use ir::data_provider::LLVMIRDataProvider;
use ir::schema::llvm_tables_fields_names;
use ir::schema::llvm_tables_fields_types;

pub mod arguments;
pub mod functions;
pub mod ir;
pub mod matchers;

fn main() {
    if cfg!(debug_assertions) {
        std::env::set_var("RUST_BACKTRACE", "1");
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }

    let args: Vec<String> = std::env::args().collect();
    let command = arguments::parse_arguments(&args);

    match command {
        Command::ReplMode(arguments) => {
            launch_llql_repl(arguments);
        }
        Command::QueryMode(query, arguments) => {
            let mut reporter = diagnostic_reporter::DiagnosticReporter::default();
            let git_repos_result = validate_files_paths(&arguments.files);
            if git_repos_result.is_err() {
                reporter.report_diagnostic(
                    &query,
                    Diagnostic::error(git_repos_result.err().unwrap().as_str()),
                );
                return;
            }

            let schema = Schema {
                tables_fields_names: llvm_tables_fields_names().to_owned(),
                tables_fields_types: llvm_tables_fields_types().to_owned(),
            };

            let std_signatures = llvm_ir_function_signatures();
            let std_functions = llvm_ir_functions();

            let aggregation_signatures = aggregation_function_signatures();
            let aggregation_functions = aggregation_functions();

            let mut env = Environment::new(schema);
            env.with_standard_functions(&std_signatures, std_functions);
            env.with_aggregation_functions(&aggregation_signatures, aggregation_functions);

            execute_llql_query(query, &arguments, &arguments.files, &mut env, &mut reporter);
        }
        Command::Help => {
            arguments::print_help_list();
        }
        Command::Version => {
            println!("GitQL version {}", env!("CARGO_PKG_VERSION"));
        }
        Command::Error(error_message) => {
            println!("{}", error_message);
        }
    }
}

fn launch_llql_repl(arguments: Arguments) {
    let mut reporter = diagnostic_reporter::DiagnosticReporter::default();
    let files = &arguments.files;
    if let Err(error) = validate_files_paths(files) {
        reporter.report_diagnostic("", Diagnostic::error(error.as_str()));
        return;
    }

    let schema = Schema {
        tables_fields_names: llvm_tables_fields_names().to_owned(),
        tables_fields_types: llvm_tables_fields_types().to_owned(),
    };

    let std_signatures = llvm_ir_function_signatures();
    let std_functions = llvm_ir_functions();

    let aggregation_signatures = aggregation_function_signatures();
    let aggregation_functions = aggregation_functions();

    let mut global_env = Environment::new(schema);
    global_env.with_standard_functions(&std_signatures, std_functions);
    global_env.with_aggregation_functions(&aggregation_signatures, aggregation_functions);

    let mut input = String::new();

    loop {
        // Render Prompt only if input is received from terminal
        if atty::is(Stream::Stdin) {
            print!("llql > ");
        }

        std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
        match std::io::stdin().read_line(&mut input) {
            Ok(buffer_length) => {
                if buffer_length == 0 {
                    break;
                }
            }
            Err(error) => {
                reporter.report_diagnostic(&input, Diagnostic::error(&format!("{}", error)));
            }
        }

        let stdin_input = input.trim();
        if stdin_input.is_empty() || stdin_input == "\n" {
            continue;
        }

        if stdin_input == "exit" {
            println!("Goodbye!");
            break;
        }

        execute_llql_query(
            stdin_input.to_owned(),
            &arguments,
            files,
            &mut global_env,
            &mut reporter,
        );

        input.clear();
        global_env.clear_session();
    }
}

fn execute_llql_query(
    query: String,
    arguments: &Arguments,
    files: &[String],
    env: &mut Environment,
    reporter: &mut DiagnosticReporter,
) {
    let front_start = std::time::Instant::now();
    let tokenizer_result = tokenizer::tokenize(query.clone());
    if tokenizer_result.is_err() {
        let diagnostic = tokenizer_result.err().unwrap();
        reporter.report_diagnostic(&query, *diagnostic);
        return;
    }

    let tokens = tokenizer_result.ok().unwrap();
    if tokens.is_empty() {
        return;
    }

    let parser_result = parser::parse_gql(tokens, env);
    if parser_result.is_err() {
        let diagnostic = parser_result.err().unwrap();
        reporter.report_diagnostic(&query, *diagnostic);
        return;
    }

    let query_node = parser_result.ok().unwrap();
    let front_duration = front_start.elapsed();

    let engine_start = std::time::Instant::now();
    let provider: Box<dyn DataProvider> = Box::new(LLVMIRDataProvider::new(files.to_vec()));
    let evaluation_result = engine::evaluate(env, &provider, query_node);
    let engine_duration = engine_start.elapsed();

    // Report Runtime exceptions if they exists
    if evaluation_result.is_err() {
        reporter.report_diagnostic(
            &query,
            Diagnostic::exception(&evaluation_result.err().unwrap()),
        );
        return;
    }

    // Render the result only if they are selected groups not any other statement
    let mut rows_count = 0;
    let engine_result = evaluation_result.ok().unwrap();
    if let SelectedGroups(mut groups) = engine_result {
        rows_count += groups.len();
        let printer: Box<dyn OutputPrinter> = match arguments.output_format {
            OutputFormat::Render => {
                Box::new(TablePrinter::new(arguments.pagination, arguments.page_size))
            }
            OutputFormat::JSON => Box::new(JSONPrinter),
            OutputFormat::CSV => Box::new(CSVPrinter),
        };
        printer.print(&mut groups);
    }

    if arguments.analysis {
        let total_time = front_duration + engine_duration;
        println!(
            "{} row in set (total: {:?}, front: {:?}, engine: {:?})",
            rows_count, total_time, front_duration, engine_duration
        );
    }
}

fn validate_files_paths(files: &[String]) -> Result<(), String> {
    for file in files {
        if !Path::new(file).exists() {
            return Err(format!("File ${} is not exists", file));
        }

        if file.ends_with(".ll") || file.ends_with(".bc") {
            continue;
        }

        return Err(format!("File ${} must end with LL or BC extension", file));
    }
    Ok(())
}
