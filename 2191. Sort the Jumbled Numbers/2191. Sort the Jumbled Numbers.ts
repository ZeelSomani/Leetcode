function sortJumbled(mapping: number[], nums: number[]): number[] 
{
    const n = nums.length;
    const f = (x: number): number => 
      {
        if (x === 0) 
        {
            return mapping[0];
        }
        let y = 0;
        for (let k = 1; x; x = (x / 10) | 0) 
        {
            const v = mapping[x % 10];
            y += v * k;
            k *= 10;
        }
        return y;
    };
    const arr: number[][] = nums.map((x, i) => [f(x), i]);
    arr.sort((a, b) => (a[0] === b[0] ? a[1] - b[1] : a[0] - b[0]));
    return arr.map(x => nums[x[1]]);
}
