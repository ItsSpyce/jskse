#include "sinks.h"

#include <RE/M/MenuOpenCloseEvent.h>

#include "log.h"

void registerEventSinks() {
  auto listener = KitchenSink::singleton();
  RE::UI::GetSingleton()->AddEventSink<RE::MenuOpenCloseEvent>(listener);
  rlog::info("Registered sink for menu open/close events.");
}

KitchenSink* KitchenSink::singleton() {
  static KitchenSink singleton;
  return &singleton;
}

RE::BSEventNotifyControl KitchenSink::ProcessEvent(
    const RE::MenuOpenCloseEvent* event,
    [[maybe_unused]] RE::BSTEventSource<RE::MenuOpenCloseEvent>* source) {
  // An example of logging:
  rlog::info("menu event: '{}' {}", event->menuName.c_str(),
             event->opening ? "opened" : "closed");
  return RE::BSEventNotifyControl::kContinue;
}
