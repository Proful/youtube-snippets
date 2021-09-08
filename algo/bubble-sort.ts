function bubbleSort(arr: number[]) {
  let isSwapped = false

  for (let i = 0; i < arr.length; i++) {
    console.log(i, arr[i], arr)

    isSwapped = false

    for (let j = 0; j < arr.length; j++) {
      // if current number is greater than next number
      // swap!
      if (arr[j] < arr[j + 1]) {
        // swap trick => [b, a] = [a, b]
        ;[arr[j], arr[j + 1]] = [arr[j + 1], arr[j]]
        isSwapped = true
      }
    }

    if (!isSwapped) {
      // console.log("Done ", i, arr[i], arr)
      break
    }
  }
}

let arr = [22, 8, 13, 32, 13]

bubbleSort(arr)
console.log("Output ", arr)
