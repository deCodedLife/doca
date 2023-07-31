import { platformBrowserDynamic } from "@angular/platform-browser-dynamic";

import { AppModule } from "./app/app.module";
import { appWindow } from "@tauri-apps/api/window";


document.body.onfullscreenchange =_ => {
    if (document.fullscreenElement) {
        appWindow.setFullscreen(true);
    } else {
        appWindow.setFullscreen(false);
    }
}


platformBrowserDynamic()
  .bootstrapModule(AppModule)
  .catch((err) => console.error(err));
