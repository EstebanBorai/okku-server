import { start as repl } from 'repl';

import log from './utils/log';
import login from './utils/login';
import me from './utils/me';
import ws from './utils/ws';

(async () => {
  const { token } = await login();
  const { user } = await me(token);
  const chatId = '85cca390-8a85-42f7-b122-262d0c924675';
  
  log.green(`Connected with success as ${user.name}`);
  log.cyan(`User ID: ${user.id}`);
  log.cyan(`Chat ID: ${chatId}`);

  const makeMessage = (message: string) => (JSON.stringify({
    inner: {
      author_id: user.id,
      chat_id: chatId,
      body: message,
      created_at: new Date(),
    }
  }));

  await ws(token, {
    onComplete() {
      log.blue('Chat finalized');
    },
    onError(e) {
      throw e;
    },
    onMessage(text) {
      console.log('Received: ' + text)
    },
    onConnect(conn) {
      repl({
        prompt: '> ',
        eval: (cmd) => {
          conn.sendUTF(makeMessage(cmd));
        }
      });
    }
  });
})();
