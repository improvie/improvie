import type { BgConfig } from 'bgutils-js';
import { BG } from 'bgutils-js';
import { Innertube } from 'youtubei.js/web';
import { Logger } from './logger';

export async function generatePoToken(): Promise<{
  poToken: string;
  visitorData: string;
}> {
// Create a barebones Innertube instance so we can get a visitor data string from YouTube.
  const innertube = await Innertube.create({
    retrieve_player: false,
  });

  const requestKey = 'O43z0dpjhgX20SCx4KAo';
  const visitorData = innertube.session.context.client.visitorData;

  if (!visitorData)
    throw new Error('Could not get visitor data');

  const bgConfig: BgConfig = {
    fetch: (input: string | URL | globalThis.Request, init?: RequestInit) => fetch(input, init),
    globalObj: globalThis,
    identifier: visitorData,
    requestKey,
  };

  const bgChallenge = await BG.Challenge.create(bgConfig);

  if (!bgChallenge)
    throw new Error('Could not get challenge');

  const interpreterJavascript = bgChallenge.interpreterJavascript.privateDoNotAccessOrElseSafeScriptWrappedValue;

  if (interpreterJavascript) {
    // eslint-disable-next-line no-eval
    eval(interpreterJavascript);
  }
  else {
    throw new Error('Could not load VM');
  }

  const poTokenResult = await BG.PoToken.generate({
    program: bgChallenge.program,
    globalName: bgChallenge.globalName,
    bgConfig,
  });

  const placeholderPoToken = BG.PoToken.generateColdStartToken(visitorData);

  Logger.debug('Session Info:', {
    visitorData,
    placeholderPoToken,
    poToken: poTokenResult.poToken,
    integrityTokenData: poTokenResult.integrityTokenData,
  });

  return {
    poToken: poTokenResult.poToken,
    visitorData,
  };
}
