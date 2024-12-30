const players = 416;
const max_score = 71617;

const data = { value: 0 };
data.prev = data;
data.next = data;

const score = {};
let cursor = data;

for (let i = 0; i <= max_score; i++) {
    if (i % 23 == 0) {
        for (let j = 0; j < 7; j++) {
            cursor = cursor.prev;
        }
        score[i % players] = score[i % players] || 0;
        score[i % players] += cursor.value + i;
        cursor.prev.next = cursor.next;
        cursor.next.prev = cursor.prev;
        cursor = cursor.next;
    } else {
        cursor = cursor.next;
        const marble = {
            prev: cursor,
            next: cursor.next,
            value: i
        }
        marble.prev.next = marble;
        marble.next.prev = marble;
        cursor = marble;
    }
}

console.log(Math.max.apply(null, Object.values(score)));