#include "bridge/papyrus.h"

#include "bridge/strings.rs.h"

static auto mcm_name = "jskse";

void papyrus::register_native_functions() {
  const auto* papyrus = SKSE::GetPapyrusInterface();
  papyrus->Register(callback);
}

bool papyrus::callback(RE::BSScript::IVirtualMachine* a_vm) {
  a_vm->RegisterFunction("StringToInt", mcm_name, string_to_int);
  return true;
}

int papyrus::string_to_int(RE::TESQuest*, RE::BSFixedString number) {
  auto numstr = std::string(number);
  // Here we call a Rust function that we've pulled in from the bridge.
  return strings::string_to_int(numstr);
}
