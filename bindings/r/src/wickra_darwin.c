/* R .Call glue for the wickra-darwin C ABI hub. */
#include <R.h>
#include <Rinternals.h>
#include <R_ext/Rdynload.h>
#include <stddef.h>
#include "wickra_darwin.h"

/* --- handle lifetime ----------------------------------------------------- */

static void wkdarwin_finalize(SEXP ext) {
    WickraDarwin *h = (WickraDarwin *)R_ExternalPtrAddr(ext);
    if (h) {
        wickra_darwin_free(h);
    }
    R_ClearExternalPtr(ext);
}

static WickraDarwin *handle_of(SEXP ext) {
    WickraDarwin *h = (WickraDarwin *)R_ExternalPtrAddr(ext);
    if (!h) {
        Rf_error("wickra-darwin: handle is closed");
    }
    return h;
}

/* --- exported .Call entries ---------------------------------------------- */

SEXP wkdarwin_version(void) {
    return Rf_mkString(wickra_darwin_version());
}

SEXP wkdarwin_new(SEXP spec_json) {
    const char *spec = CHAR(STRING_ELT(spec_json, 0));
    WickraDarwin *h = wickra_darwin_new(spec);
    if (!h) {
        Rf_error("wickra-darwin: invalid spec");
    }
    SEXP ext = PROTECT(R_MakeExternalPtr(h, R_NilValue, R_NilValue));
    R_RegisterCFinalizerEx(ext, wkdarwin_finalize, TRUE);
    UNPROTECT(1);
    return ext;
}

SEXP wkdarwin_command(SEXP ext, SEXP cmd_json) {
    WickraDarwin *h = handle_of(ext);
    const char *cmd = CHAR(STRING_ELT(cmd_json, 0));

    /* Length-out protocol: learn the length, then read into a caller buffer.
       Domain errors come back in-band as {"ok":false,...} JSON, not a negative
       code; only unusable arguments / a caught panic return < 0. */
    int len = wickra_darwin_command(h, cmd, NULL, 0);
    if (len < 0) {
        Rf_error("wickra-darwin: command failed (code %d)", len);
    }
    char *buf = (char *)R_alloc((size_t)len + 1, 1);
    wickra_darwin_command(h, cmd, buf, (size_t)len + 1);
    return Rf_mkString(buf);
}

/* --- registration -------------------------------------------------------- */

static const R_CallMethodDef CallEntries[] = {
    {"wkdarwin_version", (DL_FUNC)&wkdarwin_version, 0},
    {"wkdarwin_new", (DL_FUNC)&wkdarwin_new, 1},
    {"wkdarwin_command", (DL_FUNC)&wkdarwin_command, 2},
    {NULL, NULL, 0}};

void R_init_wickradarwin(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);
}
