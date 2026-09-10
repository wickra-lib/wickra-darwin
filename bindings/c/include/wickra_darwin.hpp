// Wickra Darwin — C++ wrapper over the C ABI.
//
// Header-only, C++17, no dependency beyond the standard library and
// `wickra_darwin.h` beside it. Link the same `wickra_darwin` library the C
// binding does.
//
// What it adds over calling the C functions directly is the handling nobody
// wants to write twice: the handle is owned and freed, the two-call length
// protocol behind `wickra_darwin_command` is done for you, and a failure comes
// back as an exception rather than a negative integer a caller can ignore.
//
//     #include <wickra_darwin.hpp>
//
//     wickra::Darwin darwin(R"({"population":32,"generations":5,...})");
//     std::string report = darwin.command(R"({"cmd":"evolve","data":{...}})");
//
// The darwin is data-driven, so this wrapper deliberately stops at strings:
// the spec and the report are JSON, and which JSON library a caller uses is
// their choice, not this header's.

#ifndef WICKRA_DARWIN_HPP
#define WICKRA_DARWIN_HPP

#include <cstddef>
#include <cstdint>
#include <stdexcept>
#include <string>
#include <utility>

#include "wickra_darwin.h"

namespace wickra {

/// Thrown when the library rejects a spec or a command.
class DarwinError : public std::runtime_error {
 public:
  explicit DarwinError(const std::string& what) : std::runtime_error(what) {}
};

/// An owning handle to a darwin built from a spec.
///
/// Move-only, because the underlying handle is a unique resource: copying it
/// would free the same pointer twice.
class Darwin {
 public:
  /// Build a darwin from a spec JSON string.
  ///
  /// Throws `DarwinError` if the spec is not valid JSON or not a valid spec.
  explicit Darwin(const std::string& spec_json)
      : handle_(wickra_darwin_new(spec_json.c_str())) {
    if (handle_ == nullptr) {
      throw DarwinError("wickra_darwin_new rejected the spec");
    }
  }

  ~Darwin() { wickra_darwin_free(handle_); }

  Darwin(const Darwin&) = delete;
  Darwin& operator=(const Darwin&) = delete;

  Darwin(Darwin&& other) noexcept : handle_(other.handle_) {
    other.handle_ = nullptr;
  }

  Darwin& operator=(Darwin&& other) noexcept {
    if (this != &other) {
      wickra_darwin_free(handle_);
      handle_ = other.handle_;
      other.handle_ = nullptr;
    }
    return *this;
  }

  /// Apply a command JSON and return the response JSON.
  ///
  /// The C entry point writes into a caller buffer and reports the length it
  /// needed, so this asks for the length first and then reads. A command the
  /// library understands but cannot carry out answers in band with
  /// `{"ok":false,"error":...}`; a negative return is a failure of the call
  /// itself and becomes an exception.
  std::string command(const std::string& cmd_json) {
    const std::int32_t needed =
        wickra_darwin_command(handle_, cmd_json.c_str(), nullptr, 0);
    if (needed < 0) {
      throw DarwinError("wickra_darwin_command failed with code " +
                          std::to_string(needed));
    }

    std::string out(static_cast<std::size_t>(needed), '\0');
    // The C side writes a trailing NUL, so the buffer has to hold one more byte
    // than the response itself.
    const std::int32_t written = wickra_darwin_command(
        handle_, cmd_json.c_str(), out.data(),
        static_cast<std::uintptr_t>(out.size()) + 1);
    if (written < 0) {
      throw DarwinError("wickra_darwin_command failed with code " +
                          std::to_string(written));
    }
    if (written != needed) {
      // The response changed length between the two calls, which cannot happen
      // for a handle only this thread is using. Saying so is better than
      // returning a string that is half of one answer and half of another.
      throw DarwinError("wickra_darwin_command length changed between calls");
    }
    return out;
  }

  /// The library version.
  static std::string version() { return std::string(wickra_darwin_version()); }

 private:
  WickraDarwin* handle_;
};

}  // namespace wickra

#endif  // WICKRA_DARWIN_HPP
