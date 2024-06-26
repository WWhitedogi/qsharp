// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

import { log, TargetProfile } from "qsharp-lang";
import * as vscode from "vscode";
import { isQsharpDocument } from "./common";
import {
  getEnableAdaptiveProfile,
  getTarget,
  getTargetFriendlyName,
  setTarget,
} from "./config";

export function activateTargetProfileStatusBarItem(): vscode.Disposable[] {
  const disposables = [];

  disposables.push(registerTargetProfileCommand());

  const statusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    200,
  );
  disposables.push(statusBarItem);

  statusBarItem.command = "qsharp-vscode.setTargetProfile";

  disposables.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      if (editor && isQsharpDocument(editor.document)) {
        refreshStatusBarItemValue();
      } else if (editor?.document.uri.scheme !== "output") {
        // The output window counts as a text editor.
        // Avoid hiding the status bar if the focus is
        // on the output window.
        // https://github.com/Microsoft/vscode/issues/58869

        // Hide the status bar if we switched away from a
        // Q# document.
        statusBarItem.hide();
      }
    }),
  );

  disposables.push(
    vscode.workspace.onDidChangeConfiguration((event) => {
      if (
        vscode.window.activeTextEditor &&
        isQsharpDocument(vscode.window.activeTextEditor.document) &&
        event.affectsConfiguration("Q#.targetProfile")
      ) {
        refreshStatusBarItemValue();
      }
    }),
  );

  if (
    vscode.window.activeTextEditor &&
    isQsharpDocument(vscode.window.activeTextEditor.document)
  ) {
    refreshStatusBarItemValue();
  }

  function refreshStatusBarItemValue() {
    // The target profile setting is a "window" scoped setting,
    // meaning it can't be set on a per-folder basis. So we don't
    // need to pass a specific scope here to retrieve the document
    // value - we just use the workspace setting.
    // VS Code will return the default value defined by the extension
    // if none was set by the user, so targetProfile should always
    // be a valid string.
    const targetProfile = getTarget();

    statusBarItem.text = getTargetFriendlyName(targetProfile);
    statusBarItem.tooltip = new vscode.MarkdownString(`## Q# target profile
  The target profile determines the set of operations that are available to Q#
  programs, in order to generate valid QIR for the target platform. For more
  details see <https://aka.ms/qdk.qir>.`);
    statusBarItem.show();
  }

  return disposables;
}

function registerTargetProfileCommand() {
  return vscode.commands.registerCommand(
    "qsharp-vscode.setTargetProfile",
    async () => {
      const target = await vscode.window.showQuickPick(
        getTargetProfiles().map((profile) => ({
          label: profile.uiText,
        })),
        { placeHolder: "Select the QIR target profile" },
      );

      if (target) {
        setTarget(getTargetProfileSetting(target.label));
      }
    },
  );
}

const targetProfiles = [
  { configName: "base", uiText: "Q#: QIR base" },
  { configName: "quantinuum", uiText: "Q#: QIR Quantinuum" },
  { configName: "unrestricted", uiText: "Q#: unrestricted" },
];

function getTargetProfiles(): {
  configName: string;
  uiText: string;
}[] {
  const allow_quantinuum = getEnableAdaptiveProfile();
  if (allow_quantinuum) {
    return targetProfiles;
  } else {
    return targetProfiles.filter(
      (profile) => profile.configName !== "quantinuum",
    );
  }
}

function getTargetProfileSetting(uiText: string): TargetProfile {
  switch (uiText) {
    case "Q#: QIR base":
      return "base";
    case "Q#: QIR Quantinuum":
      return "quantinuum";
    case "Q#: unrestricted":
      return "unrestricted";
    default:
      log.error("invalid target profile found");
      return "unrestricted";
  }
}
