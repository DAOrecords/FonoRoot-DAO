import React from 'react';
import Nav from '../Nav';


export default function TestPageSix() {
  return (
    <>
      <Nav />
      <main>
        <h2>Alter Revenue Table</h2>

        <section>
          <p>{"Alter the RevenueTable, or the Price"}</p>
          <p>{"Deleting the Price would deactivate the NFT ("}<code>{"buy()"}</code>{" action would give error)."}</p>
        </section>
      </main>
    </>
  )
}
