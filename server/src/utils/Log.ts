const {Signale} = require('signale');


export class Log {


    private static logger: any;
    private static options = {
        disabled: false,
        interactive: false,
        logLevel: 'info',
        secrets: [],
        stream: process.stdout,
        types: {
            remind: {
                badge: '**',
                color: 'yellow',
                label: 'reminder',
                logLevel: 'info'
            },
            socket: {
                badge: '⏰',
                color: 'magenta',
                label: 'santa',
                logLevel: 'info'
            }
        }
    };

    static init()  {
        this.logger = new Signale(this.options)
        this.logger.remind('test')
    }

    static getInstance() {
        return this.logger
    }

}
