🔹ExplanationTaxStrategy: Trait that defines the tax calculation strategy.
  Vat, IncomeTax, NoTax: concrete implementations of the strategy.Invoice (context): receives the strategy in its constructor (Box<dyn TaxStrategy>) and uses it to calculate the total.
  Dynamic polymorphism: we can change the strategy at runtime without modifying the invoice logic.
  
✅ AdvantagesFlexible code: you can add new tax strategies without touching Invoice.
  Practical use: useful for financial calculations, business rules, validations, interchangeable algorithms.Fits perfectly in Rust with traits and Box<dyn Trait>.

👌 LifeTime: Step by step:Box<dyn TaxStrategy + 'a>dyn TaxStrategy is a trait object (dynamic polymorphism).+ 'a means: "this object inside the box must live at least as long as 'a".This ensures that we will not use a TaxStrategy that disappears before we use it.struct Invoice<'a>We are telling Invoice: "Hold inside itself a strategy that lives at least as long as 'a".In this way, the life of the Invoice is tied to the life of the strategy it contains.impl<'a> Invoice<'a>This indicates that all methods of Invoice also use that lifetime 'a.
