import {Injectable} from '@angular/core';
import {SSE} from "./SSE/sse";
import {cashboxID, env} from "../env/environment";
import {Api} from "./API/api";
import {HttpClient} from "@angular/common/http";


@Injectable({
  providedIn: 'root'
})
export class BackgroundService {

  api: Api = new Api( this.http )

  processRequest( data: object ) {
    console.log( data )
  }

  constructor(
      private http: HttpClient,
  ) {
    let sse = new SSE( env.API, env.DEV_REQUEST )
    sse.dataReceived.subscribe( response => this.processRequest( response ) )

    cashboxID.then( id => {
      let request = env.GET_TRANSACTIONS
      request.data.cashbox_id = id
      this.api.sendRequest( request ).subscribe( response => {
        console.log( response )
      } )
    } )
  }
}
