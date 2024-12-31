#pragma once

// Pull in Cxx's rust types (not the plugin types).
#include "rust/cxx.h"

// Sorry about the file name.
namespace util
{
	void notify_player(const std::string& message);
	rust::String lookup_translation(const std::string& key);

	std::vector<uint8_t> chars_to_vec(const char* input);
	std::string name_as_utf8(const RE::TESForm* form);
}
