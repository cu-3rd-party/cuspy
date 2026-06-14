import { createClient, type CallOptions } from '@connectrpc/connect';
import { Greeter } from '$lib/proto/helloworld/helloworld_pb';
import { transport } from './transport';

export const greeterClient = createClient(Greeter, transport);

export function sayHello(name: string, options?: CallOptions) {
	return greeterClient.sayHello({ name }, options);
}
