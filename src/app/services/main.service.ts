import { Injectable } from '@angular/core';
import { DomSanitizer, SafeResourceUrl } from '@angular/platform-browser';
import { invoke } from '@tauri-apps/api/tauri'

@Injectable({
  providedIn: 'root'
})
export class MainService {
  app_url: string
  safe_url: SafeResourceUrl

  constructor(
    private sanitizer: DomSanitizer
  ) {
    invoke( 'get_crm_url' ).then( (url: string) => {
      this.app_url = url
      this.safe_url = this.sanitizer.bypassSecurityTrustResourceUrl( url )
    } )
  }
}
