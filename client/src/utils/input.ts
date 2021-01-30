import { prompt } from 'inquirer';

import log from './log';

export default (qst: string): Promise<string> => new Promise((resolve) => {
  prompt([
    {
      name: 'question',
      'type': 'input',
    }
  ]).then((answers) => resolve(answers.question)).catch((err) => {
    log.red('Inquirer Error: ' + err.toString());
  });
});
