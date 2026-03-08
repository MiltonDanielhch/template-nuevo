declare global {
  interface Window {
    htmx: typeof import("htmx.org");
  }
}

declare namespace htmx {
  const version: string;
  const config: {
    defaultSwapStyle: string;
    defaultSettleDelay: number;
    historyCacheSize: number;
    refreshOnHistoryMiss: boolean;
  };

  function ajaxPrefilter(options: any): void;
  function on(event: string, selector: string, handler: (e: Event) => void): void;

  type HtmxEvent = Event & {
    detail: {
      target: HTMLElement;
      xhr: XMLHttpRequest;
      request: XMLHttpRequest;
    };
  };
}

export {};
