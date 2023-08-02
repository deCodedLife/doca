import { NgModule } from "@angular/core";
import { BrowserModule } from "@angular/platform-browser";

import { AppComponent } from "./app.component";
import { MainService } from "./services/main.service";



@NgModule({
  declarations: [AppComponent],
  imports: [
      BrowserModule
  ],
  providers: [],
  bootstrap: [AppComponent],
})

export class AppModule {
  constructor(
      private main: MainService
  ) {
  }
}
