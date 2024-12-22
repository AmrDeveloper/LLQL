use std::fs;
use std::io;
use std::io::IsTerminal;
use std::path::Path;

use arguments::Arguments;
use arguments::Command;
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
use gitql_parser::tokenizer::Tokenizer;
use gitql_std::aggregation::aggregation_function_signatures;
use gitql_std::aggregation::aggregation_functions;
use ir::data_provider::LLVMIRDataProvider;
use ir::module_parser::parse_llvm_modules;
use ir::schema::llvm_tables_fields_names;
use ir::schema::llvm_tables_fields_types;
use lineeditor::LineEditorResult;

pub mod arguments;
pub mod functions;
pub mod ir;
pub mod line_editor;
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
            launch_llql_repl(&arguments);
        }
        Command::ScriptMode(script_file, arguments) => {
            let mut reporter = diagnostic_reporter::DiagnosticReporter::default();
            let files_validation_result = validate_files_paths(&arguments.files);
            if files_validation_result.is_err() {
                reporter.report_diagnostic(
                    "",
                    Diagnostic::error(files_validation_result.err().unwrap().as_str()),
                );
                return;
            }

            if let Err(parse_modules_error) = parse_llvm_modules(&arguments.files) {
                reporter.report_diagnostic("", Diagnostic::error(parse_modules_error.as_str()));
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

            let query =
                fs::read_to_string(script_file).expect("Should have been able to read the file");

            let provider: Box<dyn DataProvider> =
                Box::new(LLVMIRDataProvider::new(arguments.files.clone()));

            execute_llql_query(query, &arguments, &mut env, &provider, &mut reporter);
        }
        Command::QueryMode(query, arguments) => {
            let mut reporter = diagnostic_reporter::DiagnosticReporter::default();

            if let Err(validate_files_errors) = validate_files_paths(&arguments.files) {
                reporter
                    .report_diagnostic(&query, Diagnostic::error(validate_files_errors.as_str()));
                return;
            }

            if let Err(parse_modules_error) = parse_llvm_modules(&arguments.files) {
                reporter.report_diagnostic("", Diagnostic::error(parse_modules_error.as_str()));
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

            let provider: Box<dyn DataProvider> =
                Box::new(LLVMIRDataProvider::new(arguments.files.clone()));

            execute_llql_query(query, &arguments, &mut env, &provider, &mut reporter);
        }
        Command::Help => {
            arguments::print_help_list();
        }
        Command::Version => {
            println!("LLQL version {}", env!("CARGO_PKG_VERSION"));
        }
        Command::Error(error_message) => {
            println!("{}", error_message);
        }
    }
}

fn launch_llql_repl(arguments: &Arguments) {
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

    if let Err(parse_modules_error) = parse_llvm_modules(files) {
        reporter.report_diagnostic("", Diagnostic::error(parse_modules_error.as_str()));
        return;
    }

    let provider: Box<dyn DataProvider> = Box::new(LLVMIRDataProvider::new(files.to_vec()));

    // Launch the right line editor if the flag is enabled
    // Later this line editor will be the default editor
    if arguments.enable_line_editor {
        let mut line_editor = line_editor::create_new_line_editor();
        loop {
            if let Ok(LineEditorResult::Success(input)) = line_editor.read_line() {
                println!();

                if input.is_empty() || input == "\n" {
                    continue;
                }

                if input == "exit" {
                    break;
                }

                execute_llql_query(
                    input.to_owned(),
                    arguments,
                    &mut global_env,
                    &provider,
                    &mut reporter,
                );

                global_env.clear_session();
            }
        }
        return;
    }

    let mut input = String::new();
    loop {
        let stdin = io::stdin();

        // Render Prompt only if input is received from terminal
        if stdin.is_terminal() {
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
            arguments,
            &mut global_env,
            &provider,
            &mut reporter,
        );

        input.clear();
        global_env.clear_session();
    }
}

#[allow(clippy::borrowed_box)]
fn execute_llql_query(
    query: String,
    arguments: &Arguments,
    env: &mut Environment,
    provider: &Box<dyn DataProvider>,
    reporter: &mut DiagnosticReporter,
) {
    let front_start = std::time::Instant::now();
    let tokenizer_result = Tokenizer::tokenize(query.clone());
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
    let evaluation_result = engine::evaluate(env, provider, query_node);
    let engine_duration = engine_start.elapsed();

    // Report Runtime exceptions if they exists
    if evaluation_result.is_err() {
        let exception = Diagnostic::exception(&evaluation_result.err().unwrap());
        reporter.report_diagnostic(&query, exception);
        return;
    }

    // Render the result only if they are selected groups not any other statement
    let printer: Box<dyn OutputPrinter> = match arguments.output_format {
        OutputFormat::Render => {
            Box::new(TablePrinter::new(arguments.pagination, arguments.page_size))
        }
        OutputFormat::JSON => Box::new(JSONPrinter),
        OutputFormat::CSV => Box::new(CSVPrinter),
    };

    // Render the result only if they are selected groups not any other statement
    let evaluations_results = evaluation_result.ok().unwrap();
    for evaluation_result in evaluations_results {
        let mut rows_count = 0;
        if let SelectedGroups(mut groups) = evaluation_result {
            if !groups.is_empty() {
                rows_count += groups.groups[0].len();
                printer.print(&mut groups);
            }
        }

        if arguments.analysis {
            let total_time = front_duration + engine_duration;
            println!(
                "{} row in set (total: {:?}, front: {:?}, engine: {:?})",
                rows_count, total_time, front_duration, engine_duration
            );
        }
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
