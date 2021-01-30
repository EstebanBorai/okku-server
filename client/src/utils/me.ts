import got from 'got';

export interface MeResponse {
  user: {
    id: string;
    name: string;
  },
  profile: null;
};

export default async (token: string): Promise<MeResponse> => {
  const me = await got.get<MeResponse>('http://0.0.0.0:3000/api/v1/auth/me', {
    headers: {
      authorization: `Bearer ${token}`,
    }
  }).json();

  return me as MeResponse;
}
