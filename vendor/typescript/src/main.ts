export interface Config {
  server: {
    host: string;
    port: number;
  };
}

export interface EventHandlers {
  onMessage(message: Message): void;
  onConnect(ev: Event): void;
  onDisconnect(ev: CloseEvent): void;
  onError(ev: Event): void;
}

export interface Chat {
  id: string;
  messages: Message[];
  participants_ids: string[];
}

export interface User {
  id: string;
  name: string;
}

export interface Message {
  id: string;
  body: string;
  chat: Chat;
  author: User;
  created_at: Date;
}

export interface InputProto {
  inner: {
    author_id: string;
    chat_id: string;
    body: string;
    created_at: Date;
  };
}

export default class XeedClient {
  private token: string;
  private webSocket: WebSocket | null;
  private config: Config;
  private sentMessages: InputProto[];
  private receivedMessages: Message[];
  private eventHandlers: EventHandlers;
  private user: User | null;

  constructor(
    { host, port }: { host: string; port: number },
    token: string,
    eventHandlers: EventHandlers,
  ) {
    this.token = token;
    this.webSocket = null;
    this.receivedMessages = [];
    this.sentMessages = [];
    this.eventHandlers = eventHandlers;
    this.user = null;
    this.config = {
      server: {
        host,
        port,
      },
    };
  }

  public async connect(): Promise<void> {
    const {
      config: {
        server: { host, port },
      },
      token,
    } = this;

    this.user = await this.me();

    const ws = new WebSocket(
      `ws://${host}:${port}/api/v1/chats?token=${token}`,
    );

    ws.onclose = this.eventHandlers.onDisconnect;
    ws.onerror = this.eventHandlers.onError;
    ws.onmessage = this.handleIncomingMessage;
    ws.onopen = this.eventHandlers.onConnect;

    this.webSocket = ws;
  }

  public disconnect(): void {
    // refer to close codes: https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent#Status_codes
    this.webSocket?.close(1000);
    this.webSocket = null;
  }

  public sendUtf8(chatId: string, text: string): void {
    const message: InputProto = {
      inner: {
        author_id: this.user?.id as string,
        chat_id: chatId,
        body: text,
        created_at: new Date(),
      },
    };

    this.sentMessages.push(message);
    this.webSocket?.send(JSON.stringify(message));
  }

  private handleIncomingMessage(message: MessageEvent<string>): void {
    const parsedMessage: Message = JSON.parse(message.data);

    this.receivedMessages.push(parsedMessage);
    this.eventHandlers.onMessage(parsedMessage);
  }

  private async me(): Promise<{ id: string; name: string }> {
    const url = new URL(
      `http://${this.config.server.host}:${this.config.server.port}/api/v1/auth/me`,
    );
    const headers = new Headers();

    headers.append('Authorization', 'Bearer ' + this.token);

    const response = await fetch(url.toString(), {
      headers,
      mode: 'cors',
    });

    if (response.status !== 200) {
      throw new Error(
        'FetchMeError: ' + response.status + ' ' + response.statusText,
      );
    }

    const json = await response.json();

    return json.user;
  }
}

(window as any).XeedClient = XeedClient;
