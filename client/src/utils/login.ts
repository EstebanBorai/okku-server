import got from 'got';

export default async (): Promise<{ token: string; }> => {
  const [,, username, password] = process.argv;

  if (!username || !password) {
    throw new Error('Missing credentials. Execute script providing username and password as params');
  }

  const { token } = await got<{ token: string; }>('http://0.0.0.0:3000/api/v1/auth/login', {
    username,
    password,
  }).json();

  return {
    token,
  }
}
