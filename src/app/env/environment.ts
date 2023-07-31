import { invoke } from "@tauri-apps/api/tauri";

export let cashboxID = invoke<string>( 'get_cashbox_id', {} )

export const env = {
    API: "https://dev.docacrm.com",
    DEV_REQUEST: {
        object: "dev",
        command: "any",
        data: {}
    },
    GET_TRANSACTIONS: {
        object: "atol",
        command: "get-transactions",
        data: {
            cashbox_id: ""
        }
    }
}

// greetingMessage = "";
//
// greet(event: SubmitEvent, name: string): void {
//   event.preventDefault();
//
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   invoke<string>("greet", { name }).then((text) => {
//     this.greetingMessage = text;
//   });
// }