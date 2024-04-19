# chemical-equation-balancer
Takes a chemical equation and balances it

### User Stories
- [ ] As a user, I want to be able to input a chemical equation
- [ ] As a user, I want to get the most simplified, balanced chemical equation
- [ ] As a user, I want to enter the number of moles of a molecule
- [ ] As a user, I want to see how many moles are produced by another molecule
- [ ] As a user, I want to see the limiting reactant

##### As a user, I want to be able to input a chemical equation
- Receive a molecule, it's formula, and amount
- Receive the molecules that are reactants
- Receive the molecules that are products
- Assign elements to their respective molecule
- Make sure that changes to the molecules reflect upon the elements
  - Increasing the number of molecules multiplies the amount of elements

##### As a user, I want to get the most simplified, balanced chemical equation
- Check if the total number of reactant elements are equal to the total number of product elements
- If elements are of smaller amount on reactants side, find which element is smaller than the product side
- Find molecules that contain that element
- Compare the other elements of those molecules and see if they are balanced
- Find which molecule gets closer to balance and increase that molecule
- Recheck balance
- If balanced, see if the amount of molecules can be simplified equally for all molecules
- Display balanced equation

##### As a user, I want to enter the number of grams of a molecule
- First, get balanced equation
- Save grams of reactant to the molecule specified

##### As a user, I want to see how many moles are produced by another molecule
- Calculate molar mass for each molecule by grabbing data from atomic mass database
- Calculate how many grams of a product molecule there are based on the reactant grams entered by user
  - Divide grams by molar mass
  - Grab amount of reactant molecule and product molecule from balanced equation
  - Multiply the moles of reactant by (amount of product / amount of reactant)
  - Multiply the resulting moles of the product by the products molar mass to get grams of product
- Display grams of product

##### As a user, I want to see the limiting reactant
- The limiting reactant is the substance that produces the least amount of product
- Receive the amount of grams for each reactant
- Compare the number of substance produced by all the reactants
- The one that produces the least is the limiting reactant