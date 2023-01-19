import { utils } from 'near-api-js';
import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { buyNFT, getCatalogue, getListOfIncomeTables, getNftMetadata, getPrice, getSingleIncomeTable } from '../utils';


export default function Buy() {
  const [incomeTables, setIncomeTables] = useState([]);
  const [metaList, setMetaList] = useState([]);

  // Get the list of IncomeTables
  useEffect(async () => {
    const list = await getListOfIncomeTables(0, 1000);
    setIncomeTables(list);
  }, []);

  // This useEffect is for filling in all the info in the list, like Title, Contract, RootId
  useEffect(async () => {
    console.table("Catalogue: ", incomeTables);
    if (incomeTables.length === 0) return;
    
    Promise.all(
      incomeTables.map(async (incomeTable) => {
        try {
          const meta = await getNftMetadata(incomeTable[1].contract, incomeTable[1].root_id);
          /*const catalogue = await getCatalogue(incomeTable[1].owner);
          const ind = catalogue.findIndex((catEntry) => catEntry[0] === incomeTable[0]);
          const catalogueEntry = catalogue[ind][1];
          const price = catalogueEntry.price;*/
          return { meta: meta, contract: incomeTable[1].contract, root_id: incomeTable[1].root_id, price: incomeTable[1].price};
        } catch (err) {
          return console.error(`getNftMetadata errored, contract is ${incomeTable[1].contract} and rootId is ${incomeTable[1].root_id}`, err);
        }
      })
    )
    .then((result) => {
      console.log("The Promise All result is: ", result);
      let tempArray = result.map((resultLine, index) => {
        return {
          treeIndex: incomeTables[index][0],
          title: resultLine.meta.title,
          contract: resultLine.contract,
          rootId: resultLine.root_id,
          price: resultLine.price
        }
      });
      setMetaList(tempArray);
    })
    .catch((err) => console.error("The Promise All errored: ", err));
  }, [incomeTables]);


  return (
    <>
      <Nav />
      <main>
        <h2>Buy an NFT</h2>

        <section>
        <ul className="revenueList">
            {metaList.map((entry, index) => (
              <li key={entry.treeIndex} className="buyEntry">
                <button className="buyButton" onClick={() => buyNFT(entry.rootId, entry.contract, entry.price)}>
                  <p>{entry.title}</p>
                  <p>{utils.format.formatNearAmount(entry.price)}{" NEAR"}</p>
                </button>
              </li>
            ))}
          </ul>
        </section>

      </main>
    </>
  )
}
