🔹ExplanationTaxStrategy: Trait that defines the tax calculation strategy.
  Vat, IncomeTax, NoTax: concrete implementations of the strategy.Invoice (context): receives the strategy in its constructor (Box<dyn TaxStrategy>) and uses it to calculate the total.
  Dynamic polymorphism: we can change the strategy at runtime without modifying the invoice logic.
  
✅ AdvantagesFlexible code: you can add new tax strategies without touching Invoice.
  Practical use: useful for financial calculations, business rules, validations, interchangeable algorithms.Fits perfectly in Rust with traits and Box<dyn Trait>.
