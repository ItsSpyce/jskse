#pragma once

namespace cosave
{
	void initialize_cosaves();
	void on_game_saved(SKSE::SerializationInterface* cosave);
	void on_revert(SKSE::SerializationInterface* cosave);
	void on_game_loaded(SKSE::SerializationInterface* cosave);
}
