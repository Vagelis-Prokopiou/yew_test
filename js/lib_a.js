export function getIsoDate() {
    return (new Date()).toISOString();
}

export class MyClass {
    constructor(n) {
        this._number = n;
    }

    get number() {
        return this._number;
    }

    set number(n) {
        return this._number = n;
    }

    render() {
        return `My number is: ${this.number}`;
    }
}