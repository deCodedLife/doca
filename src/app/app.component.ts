import { Component } from "@angular/core";
import { MainService } from "./services/main.service";

@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  styleUrls: ["./app.component.css"],
})
export class AppComponent {
  constructor ( public main: MainService ) {}
}
