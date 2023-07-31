import {HttpClient, HttpClientModule} from "@angular/common/http";
import {env} from "../../env/environment";
import {Injectable} from "@angular/core";

export class Response
{
    status: number = 0
    data: any | object
    detail: any | object
}

@Injectable({
    providedIn: 'root'
})
export class Api {

    public sendRequest( body: object ) {
        return this.http.post<Response>( env.API, body )
    }

    constructor(
        private http: HttpClient
    ) {
    }

}