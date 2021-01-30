import * as websocket from 'websocket';

import log from './log';

import type { connection as Connection, IMessage } from 'websocket';

export interface WSConfig {
  onConnect(conn: Connection): void;
  onMessage(message: string): void;
  onError(e: Error): void;
  onComplete(): void;
}

export default async (token: string, cb: WSConfig): Promise<void> => {
  const WebSocketClient = new websocket.client();

  WebSocketClient.on('connectFailed', (e) => {
    log.red('Connect Error: ' + e.toString());
  });

  WebSocketClient.on('connect', (conn: Connection) => {
    log.green('Authenticated with Comlink WebSocket!');

    conn.on('error', (e) => {
      log.red('Connection Error: ' + e.toString());
      cb.onError(e);
    });

    conn.on('close', (code: number) => {
      log.yellow('Connection closed with code: ' + code);
      cb.onComplete();
    });

    conn.on('message', (message: IMessage) => {
      if (message.type === 'utf-8') {
        cb.onMessage(message.utf8Data);
      }
    });

    cb.onConnect(conn);
  });

  WebSocketClient.connect('http://0.0.0.0:3000/api/v1/chats?token=' + token);
}
