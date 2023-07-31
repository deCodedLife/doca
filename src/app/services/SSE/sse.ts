import {EventEmitter} from "@angular/core";

export class SSE {

    dataReceived: EventEmitter<object> = new EventEmitter<object>()

    async loadData() {
        const response = await fetch( this.url, {
            method: 'POST',
            headers: {
                "Content-type": "application/json",
                "Accept": "text/event-stream"
            },
            body: JSON.stringify( this.body )
        } )

        if ( response.body == null ) return
        const reader = response.body.getReader()

        while ( true ) {
            const { value, done } = await reader.read()
            if ( done ) break;

            let event = JSON.parse( new TextDecoder().decode( value ) )
            this.dataReceived.emit( event )
        }
    }

    constructor(
        private url: string,
        private body: object
    ) {
       let _ = this.loadData()
    }
}