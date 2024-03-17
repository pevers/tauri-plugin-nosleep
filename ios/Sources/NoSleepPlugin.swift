import SwiftRs
import Tauri
import UIKit
import WebKit

class NoSleepPlugin: Plugin {
  @objc public func load(_ invoke: Invoke) throws {
    // no-op
  }
}

// TODO: Implement the NoSleep functionality directly in Swift!

@_cdecl("init_plugin_nosleep")
func initPlugin() -> Plugin {
  return NoSleepPlugin()
}
