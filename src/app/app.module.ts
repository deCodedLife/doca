import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import {BackgroundService} from "./services/background.service";
import {HttpClientModule} from "@angular/common/http";

import { appWindow } from "@tauri-apps/api/window";

document.addEventListener("fullscreenchange", () => {
    if (document.fullscreenElement) {
        appWindow.setFullscreen(true);
    } else {
        appWindow.setFullscreen(false);
    }
});

@NgModule({
  declarations: [AppComponent],
  imports: [
      BrowserModule,
      HttpClientModule
  ],
  providers: [],
  bootstrap: [AppComponent],
})

export class AppModule {
  constructor(
      private background: BackgroundService
  ) {
      // appWindow.setFullscreen( true )
  }
}
