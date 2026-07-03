export function sleep(n: number) {
    return new Promise((r, x) => {
        setTimeout(r, n);
    })
}