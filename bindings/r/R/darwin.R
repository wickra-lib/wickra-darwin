#' The wickra-darwin library version.
#' @return A version string.
#' @export
wkdarwin_version <- function() {
  .Call(C_wkdarwin_version)
}

#' Build a search handle from a spec JSON.
#' @param spec_json An `EvolveSpec` JSON string (`"{}"` defers configuration to a
#'   later `set_spec` command).
#' @return A `wickra_darwin` handle (an external pointer).
#' @export
wkdarwin_new <- function(spec_json) {
  .Call(C_wkdarwin_new, spec_json)
}

#' Apply a command JSON and return the resulting response JSON.
#' @param darwin A search handle from [wkdarwin_new()].
#' @param cmd_json A command JSON string (`set_spec`, `evolve`, `best`,
#'   `version`).
#' @return The response as a JSON string.
#' @export
wkdarwin_command <- function(darwin, cmd_json) {
  .Call(C_wkdarwin_command, darwin, cmd_json)
}
