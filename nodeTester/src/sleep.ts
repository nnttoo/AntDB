export function sleep(n: number) {
    return new Promise((r, x) => {
        setTimeout(r, n);
    })
}

export interface  TestMethod {
    name : string;
    success : boolean;  
    errror? : string;
    onTest() : Promise<any>;
}
 