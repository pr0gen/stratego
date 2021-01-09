
export class Socket {

     private static socket: any;

    static setSocket(socket:any) {
        this.socket = socket
    }

    static getSocket(){
        return this.socket
    }

}
