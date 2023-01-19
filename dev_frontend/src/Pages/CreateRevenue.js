import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { actOnProposal, createRevenue, getCatalogue, getLastProposalId, getListOfProposals, getNftMetadata, getSingleIncomeTable } from '../utils';
import xButton from '../assets/xButton.svg';
import blackXButton from '../assets/blackXButton.svg';


export default function TestPageFive() {
  const [withoutRevenueTable, setWithoutRevenueTable] = useState([]);
  const [metaList, setMetaList] = useState([]);
  const [selected, setSelected] = useState(null);
  const [message, setMessage] = useState("");
  const [proposalId, setProposalId] = useState(null);
  const [revenues, setRevenues] = useState([                                     // Will contain objects of the format { account: "alice.near", percent: 2000 }
    { account: "daorecords.sputnik-dao.near", percent: 1500 },
    { account: "recordpooldao.sputnik-dao.near", percent: 500 }
  ]);  
  const [price, setPrice] = useState(0);
  const [readyForVote, setReadyForVote] = useState(false);

  async function initiateCreateRevenue() {
    if (selected === null)  { console.log("Nothing is selected."); return; }

    const { treeIndex, contract, rootId, title} = metaList[selected];
    const lastProposalId = await getLastProposalId();
    localStorage.setItem("last_proposal_id", lastProposalId);
    localStorage.setItem("last_root_id", rootId);

    let rTable = {};
    revenues.map((entry) => {
      rTable[entry.account] = entry.percent;
      return;
    });

    const returnedProposalID = await createRevenue(rootId, contract, rTable, price);
    localStorage.setItem("last_proposal_id", returnedProposalID);
    setProposalId(returnedProposalID);
    setReadyForVote(true);
  }

  // This useEffect is setting the proposalId, for finalizing
  // There is a known bug, when the operation is not finished.
  useEffect(async () => {
    let catalogue = await getCatalogue(window.accountId);
    let emptyCatalogueEntries = catalogue.filter((entry) => {
      return entry[1] === null;
    });
    setWithoutRevenueTable(emptyCatalogueEntries);

    const index = localStorage.getItem("last_proposal_id");setProposalId(parseInt(index));
  }, []);  

  
  // This useEffect is for filling in all the info in the list, like Title, Contract, RootId
  useEffect(async () => {
    console.table("Without Revenue Table: ", withoutRevenueTable);
    if (withoutRevenueTable.length === 0) return;

    Promise.all(
      withoutRevenueTable.map((treeElement) => {
        return getSingleIncomeTable(treeElement[0])
          .then((incomeTable) => {
            return getNftMetadata(incomeTable.contract, incomeTable.root_id)
              .then((meta) => {return {meta: meta, contract: incomeTable.contract, root_id: incomeTable.root_id}})
              .catch((err) => console.error(`getNftMetadata errored, contract is ${incomeTable.contract} and rootId is ${incomeTable.root_id}`, err));
          })
          .catch((err) => console.error(`getSingleIncomeTable errored, treeIndex is ${treeElement[0]}`, err));
      })
    )
    .then((result) => {
      console.log("The Promise All result is: ", result);
      let tempArray = result.map((resultLine, index) => {
        return {
          treeIndex: withoutRevenueTable[index][0],
          title: resultLine.meta.title,
          contract: resultLine.contract,
          rootId: resultLine.root_id
        }
      });
      setMetaList(tempArray);
    })
    .catch((err) => console.error("The Promise All errored: ", err));
  }, [withoutRevenueTable]);


  // Act on proposal for CreateRevenue
  async function finaliizeCreateRevenue() {
    console.log(proposalId)
    const returnMessage = await actOnProposal(proposalId);
    setMessage(JSON.stringify(returnMessage));
  }

  function addNewRevenueEntry() {
    setRevenues((state) => {
      state.push({
        account: "",
        percent: 0,
      })
      return Object.assign([], state);
    });
  }

  function removeRevenueEntry(index) {
    if (index === 0 || index === 1) return;
    setRevenues((state) => {
      state.splice(index, 1);
      return Object.assign([], state);
    })
  }

  function changeRevenueAccount(index, newName) {
    setRevenues((state) => {
      state[index].account = newName;
      return Object.assign([], state);
    })
  }

  function changeRevenuePercent(index, newPercent) {
    if (newPercent > 100) return;
    setRevenues((state) => {
      state[index].percent = Math.ceil(newPercent*100);
      return Object.assign([], state);
    })
  }


  return (
    <>
      <Nav />
      <main>
        <h2>Create Revenue Table</h2>

        <section>
          <p>{"The revenue table is inside the Catalogues->Catalogue object. Each Artist has a Catalogue, and there is a bigger structure, Catalogues."}</p>
          <p>{"If a Song has a RevenueEntry associated with it, it is active. Otherwise the `buy()` function should give an error, because a price is not set yet for the NFT."}</p>
          <p>{"If the RevenueEntry was deleted, the NFT can not be bought."}</p>
          <p>{" "}</p>
          <p>{"We need to create a "}<code>{"revenue_table"}</code>{" and a "}<code>{"price"}</code>{" as well"}</p>
          <p>{"The empty "}<code>{"IncomeTable"}</code>{" is created in the MintRoot callback."}</p>
        </section>

        <section>
          <ul id="inProgressList">
            <p>{"These are the Catalogue Entries that does not have a RevenueTable. Click on an element to add RevenueTable"}</p>
            {metaList.map((entry, index) => {
              //const stringifiedNft = JSON.stringify(entry, null, "\t");
              return (
                <li 
                  key={entry.treeIndex} 
                  onClick={() => setSelected(index)} 
                  className="inProgressListElement"
                >
                  <p>{entry.title}</p>
                </li>
              );
            })}
          </ul>
        </section>

        {(selected !== null) && <section style={{ background: true ? "#00FF00" : "#FF0000" }}>
          <h3>{"Selected Entry: "}</h3>
          <p><strong>{"TreeIndex: "}</strong>{metaList[selected].treeIndex}</p>
          <p><strong>{"Title: "}</strong>{metaList[selected].title}</p>
          <p><strong>{"Contract: "}</strong>{metaList[selected].contract}</p>
          <p><strong>{"Root ID: "}</strong>{metaList[selected].rootId}</p>
        </section>}

        <section>
          <p>{"Here we are creating a revenue table, similar way as in the old Admin panel."}</p>
          <p>{"The revenue table will be added to the selected NFT."}</p>
          <label className="fieldName">{"Creator split  "}
            <button onClick={addNewRevenueEntry}>
              <img src={"plusButton"} alt={'+'}></img>
            </button>
          </label>
          <ul className="revenueList">
            {revenues.map((revenue, index) => (
              <li className="revenueElement" key={index}>
                <div>
                  <label htmlFor="revenueElementAddress" className="smallRevenueLabel">Address</label>
                  <input id="revenueElementAddress" type={"text"} value={revenue.account} onChange={(e) => changeRevenueAccount(index, e.target.value)} disabled={index === 0 || index === 1}></input>
                </div>
                <div>
                  <label htmlFor="revenueElementPercent" className="smallRevenueLabel">Percentage</label>
                  <input id="revenueElementPercent" type={"number"} min={0} max={100} value={revenue.percent / 100} onChange={(e) => changeRevenuePercent(index, e.target.value)} disabled={index === 0 || index === 1}></input>
                </div>
                <div className="revenueRemoveButtonContainer">
                  <label htmlFor="removeButton" className="placeholderLabel">X</label>
                  <img id="removeButton" src={(index === 0 || index === 1) ? xButton : blackXButton} alt={'X'} onClick={() => removeRevenueEntry(index)} disabled={index === 0 || index === 1}></img>
                </div>
              </li>
            ))}
          </ul>
          <div>
            <label>Price</label>
            <input type={"number"} value={price} onChange={(e) => setPrice(e.target.value)}></input>
          </div>
          <button onClick={initiateCreateRevenue}>{"Create Revenue Table"}</button>
        </section>

        <section>
          {readyForVote && <p className="finalizeMessage">You can click on finalize now!</p>}
          <p>{"We will need to act on the proposal that we've just created. See Registration"}</p>
          <p>{"Last proposal ID: "} {localStorage.getItem("last_proposal_id")}</p>
          <p>{"The ID of the proposal that we want to act on should be: "}<code>{proposalId}</code></p>
          <button onClick={finaliizeCreateRevenue}>{"Finalize Create Revenue Table"}</button>
          <p><code>{message}</code></p>
        </section>
      </main>
    </>
  )
}
