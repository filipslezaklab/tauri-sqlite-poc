import { invoke } from "@tauri-apps/api/core";

export enum TauriCommands {
  listYubiKeySerials = "list_yk",
  selectYubikey = "select_yubikey",
  encryptInput = "encrypt_input",
  decryptMessage = "decrypt_message",
}

export type CommandHook = {
  listYubikeySerials: () => Promise<string[]>;
  selectYubikey: (serial: string) => Promise<void>;
  encryptInput: (input: string) => Promise<string>;
  decryptMessage: (pin: string) => Promise<string>;
};

export const useTauriCommand = (): CommandHook => {
  return {
    listYubikeySerials: () => invoke(TauriCommands.listYubiKeySerials),
    selectYubikey: (serial) =>
      invoke(TauriCommands.selectYubikey, { serial: serial }),
    encryptInput: (input) => invoke(TauriCommands.encryptInput, { input }),
    decryptMessage: (pin) => invoke(TauriCommands.decryptMessage, { pin }),
  };
};
