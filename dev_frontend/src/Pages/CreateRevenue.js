import React from 'react';
import Nav from '../Nav';


export default function TestPageFive() {
  return (
    <>
      <Nav />
      <main>
        <h2>Create Revenue Table</h2>

        <section>
          <p>{"The revenue table is inside the Catalogues->Catalogue object. Each Artist has a Catalogue, and there is a bigger structure, Catalogues."}</p>
          <p>{"If a Song has a RevenueEntry associated with it, it is active. Otherwise the `buy()` function should give an error, because a price is not set yet for the NFT."}</p>
          <p>{"If the RevenueEntry was deleted, the NFT can not be bought."}</p>
          <p>{"On second thought, the RevenueTable and the Price is not the same, there might be more nuances to this."}</p>
        </section>
      </main>
    </>
  )
}
