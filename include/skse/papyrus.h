#pragma once

namespace papyrus
{
	void register_native_functions();
	bool callback(RE::BSScript::IVirtualMachine* a_vm);

	// A contrived example.
	int string_to_int(RE::TESQuest*, RE::BSFixedString number);
};
