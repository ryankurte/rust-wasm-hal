
/// Error for passing up underlying error kinds
// TODO: should we define error values as part of the runtime spec to simplify matching in apps?
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WasmError {
    /// Miscellaneous error from the underlying runtime
    Runtime(i32),
}
