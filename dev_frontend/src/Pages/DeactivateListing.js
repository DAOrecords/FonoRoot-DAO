import React from 'react';
import Nav from '../Nav';


export default function TestPageSeven() {
  return (
    <>
      <Nav />
      <main>
        <h2>Deactivate Listing</h2>

        <section>
          <p>{"If there is a RevenueTable associated with the NFT, it's active. If the RevenueTable does not exist yet, or it is deleted, it is not active."}</p>
          <p>{"Probably we will do this in another way, for example, we will add a field in the "}<code>{"CatalogueEntry"}</code>{" that is called "}<code>{"active"}</code></p>
        </section>
      </main>
    </>
  )
}
