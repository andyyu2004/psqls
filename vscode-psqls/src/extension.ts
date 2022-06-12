import * as vscode from "vscode";
import * as lc from "vscode-languageclient/node";

export function activate(context: vscode.ExtensionContext) {
  const opt: lc.Executable = {
    command: "psqls",
  };
  const serverOptions: lc.ServerOptions = {
    run: opt,
    debug: opt,
  };

  const clientOptions: lc.LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "sql" }],
  };

  const client = new lc.LanguageClient(
    "psqls",
    "psqls",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate() {}
