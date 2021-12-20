{
  //~ primitives
let fname: string = "Proful";
let age: number = 23;
let isMale: boolean = true;
fname += 23;
// age += "23";
// isMale = "false";
// fname = false;
console.log(fname, age, isMale);

//~ any
let address: any = { city: "Jeypore", state: "Odisha" };
address.city = false;

function sayHi(name: string): string {
  console.log("Hi " + name);
  return "Hi " + name;
}

let msg = sayHi("Proful");
console.log(msg);
// sayHi(23);

let cities = ["Jeypore", "Bhubaneswar", "Cuttack"];
console.log(cities);

cities.forEach((city) => {
  console.log(city.toUpperCase());
})

cities.forEach(function (city) {
  console.log(city.toLowerCase());
})

function displayAddress({ city, state }: { city: string, state?: string }) {
  console.log(city, state);
  // console.log(city.toUpperCase(), state.toUpperCase());
}

displayAddress({ city: "Jeypore", state: "Odisha" });
displayAddress({ city: "Jeypore" });

//~ Union Types
function fetchById(id: number | string) {
  console.log("Fetching data by id: " + id);
  // console.log(id.toUpperCase());
  if (typeof id === "string") {
    console.log(id.toUpperCase());
  } else {
    console.log(id.toFixed(2));
  }
}

fetchById(23);
fetchById("uid-23");
// fetchById(true);


//~ Type Aliases
type User = { name: string, age: number };

function displayUser(user: User) {
  console.log(user.name, user.age);
}

displayUser({ name: "Proful", age: 23 });

//~ Interfaces
interface UserInterface { name: string, age: number };

function displayUserInterface(user: UserInterface) {
  console.log(user.name, user.age);
}

displayUserInterface({ name: "Proful", age: 23 });

//~ type alias vs interface
//- very similar
//- type cannot be re-opened to add new properties
//- an interface which is always extendable
}