# Disburser

This contract is originally designed to be a registered x/feeshare withdraw address, and therefore receive fees in denominations supported by the x/feeshare module for that chain. It is designed for a team's app where ownership and contributions ideally remain unchanged from then-on. The contract is non-migratable, and ideally non-mutable upon creation.

During instantiation, a set of benefactor addresses and their associated weights--which can also be thought of as shares or % ownership of the related fee-generating contract--are provided. 

Weights are supplied between 0.01 and 1.00, or 001 and 100. Total weight must equal 1.00 or 100; The contract does not keep any funds for itself, and can only disburse funds to the provided addresses, at the initial weights set, every time.

the contract only calls 1 execute function: Disburse.

Disburse is callable by any of the benefactor addresses given upon instantiation. Disburse takes the address' current token balances, and disburses them according to the weights associated with each address. ~100% of the wallet's token balances will exit with each Disburse call to the benefactors in kind. It may be wise to have agreed times/terms of disbursement amongst benefactors.

The *disburser* contract is not intended to have admin powers over the contract for which it is registered as a x/feeshare withdraw address. Therefore, if you wish to update the parameters set at instantiation, then you must initialize another contract with different benefectors and weights. Be sure you have the keys to the admin of the fee-generating contract so you can reset the withdraw address to your new contract. Once migrated, the old address will cease to receive new feeshares, anyone will be able to call the final Disburse message, and the contract will be drained of funds or be left with indivisible dust.

While much complexity could be created, this version of the contract is meant to be simple, and rigid in its capabilities.

If you wish to expand upon this contract's capabilities, please pr or open a discussion!