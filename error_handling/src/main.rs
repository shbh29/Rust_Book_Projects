use error_handling::{error_handle, matching_for_errors, unwrap_for_errors};



fn main() {
   matching_for_errors::file_operation_with_match(); 
   unwrap_for_errors::unwrap_method_for_file_opening(); 
   error_handle::handle_error();
}
