export default class Path {
    constructor(string) {
        this.string = string;
    }

    toString() { return this.string; }

    getName() {
        return this.string.substring(this.string.lastIndexOf("/") + 1);
    }

    // TODO: Review this code.
    getResolvedPath() {
        if (this.string.substring(0, 2) == "./") {
            return location.href.substring(0, location.href.length-1) + this.string.substring(1);
        } else if (this.string[0] != "/") {
            return location.href + this.string;
        } else {
            return this.string;
        }
    }
}