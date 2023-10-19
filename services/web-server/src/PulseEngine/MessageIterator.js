import { $$asyncIterator } from 'iterall';

export default class MessageIterator {
  /**
   * Wrapper around a PulseIterator that yields pulse messages, transforming
   * them to {[eventName]: {payload, ..}} format as expected by pulse-messages
   * subscriptions
   */
  constructor(messageIterator, eventName) {
    this.eventName = eventName;
    this.messageIterator = messageIterator;
  }

  async next() {
    const { value, done } = await this.messageIterator.next();
    const event = { [this.eventName]: value };

    return { value: event, done };
  }

  return() {
    return this.messageIterator.return();
  }

  throw(error) {
    return this.messageIterator.throw(error);
  }

  [$$asyncIterator]() {
    return this;
  }
}
