import { utils } from 'near-api-js';
import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { actOnProposal, alterRevenue, getCatalogue, getLastProposalId, getNftMetadata, getSingleIncomeTable } from '../utils';
import xButton from '../assets/xButton.svg';
import blackXButton from '../assets/blackXButton.svg';


export default function TestPageSix() {
  const [catalogue, setCatalogue] = useState([]);
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

  async function initiateAlterRevenue() {
    if (selected === null)  { console.log("No entry is selected."); return; }

    const { treeIndex, contract, rootId, title} = metaList[selected];
    const lastProposalId = await getLastProposalId();
    localStorage.setItem("last_proposal_id", lastProposalId);
    // non-unique id
    localStorage.setItem("last_root_id", rootId);

    let rTable = {};
    revenues.map((entry) => {
      rTable[entry.account] = entry.percent;
      return;
    });

    const returnedProposalID = await alterRevenue(treeIndex, rTable, price);
    localStorage.setItem("last_proposal_id", returnedProposalID);
    setProposalId(returnedProposalID);
    setReadyForVote(true);
  }

  // This useEffect is for filling in all the info in the list, like Title, Contract, RootId
  useEffect(async () => {
    console.table("Catalogue: ", catalogue);
    if (catalogue.length === 0) return;

    Promise.all(
      catalogue.map((treeElement) => {
        return getSingleIncomeTable(treeElement[0])
          .then((incomeTable) => {
            return getNftMetadata(incomeTable.contract, incomeTable.root_id)
              .then((meta) => {return {meta: meta, contract: incomeTable.contract, root_id: incomeTable.root_id, price: incomeTable.price}})
              .catch((err) => console.error(`getNftMetadata errored, contract is ${incomeTable.contract} and rootId is ${incomeTable.root_id}`, err));
          })
          .catch((err) => console.error(`getSingleIncomeTable errored, treeIndex is ${treeElement[0]}`, err));
      })
    )
    .then((result) => {
      console.log("The Promise All result is: ", result);
      let tempArray = result.map((resultLine, index) => {
        return {
          treeIndex: catalogue[index][0],
          title: resultLine.meta.title,
          contract: resultLine.contract,
          rootId: resultLine.root_id,
          price: resultLine.price
        }
      });
      setMetaList(tempArray);
    })
    .catch((err) => console.error("The Promise All errored: ", err));
  }, [catalogue]);


  // This useEffect is setting the proposalId, for finalizing
  // There is a known bug, when the operation is not finished.
  useEffect(async () => {
    let cat = await getCatalogue(window.accountId);
    let nonEmptyCatalogueEntries = cat.filter((entry) => {
      return entry[1] !== null;
    });
    //setWithoutRevenueTable(nonEmptyCatalogueEntries);
    setCatalogue(nonEmptyCatalogueEntries)

    const index = localStorage.getItem("last_proposal_id");setProposalId(parseInt(index));
  }, []);

  // Load the values into the editable fields, when selection changes
  useEffect(() => {
    if (!catalogue[selected]) return;
    let rTable = [    // Will contain objects of the format { account: "alice.near", percent: 2000 }
      { account: "daorecords.soundsplash.testnet", percent: 1500 },
      { account: "recordpooldao.soundsplash.testnet", percent: 500 }
    ];
    let selectedRevenueObj = Object.assign({}, catalogue[selected][1].revenue_table);
    delete selectedRevenueObj["daorecords.soundsplash.testnet"];
    delete selectedRevenueObj["recordpooldao.soundsplash.testnet"];

    Object.keys(selectedRevenueObj).map((entry, index) => {
      rTable.push({
        account: entry,
        percent: selectedRevenueObj[entry]
      })
    });

    setRevenues(rTable);
    setPrice(utils.format.formatNearAmount(metaList[selected].price))
  }, [selected]);

  // Act on proposal for AlterRevenue
  async function finaliizeAlterRevenue() {
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
        <h2>Alter Revenue Table</h2>

        <section>
          <p>{"Alter the RevenueTable, or the Price"}</p>
          <p>{"Deleting the Price would deactivate the NFT ("}<code>{"buy()"}</code>{" action would give error)."}</p>
        </section>

        {(metaList.length > 0) && <section>
          <p>{"List of Catalogue Entries for logged in user: "}<strong>{window.accountId}</strong></p>
          <ul id="revenueTableList">
            {catalogue.map((entry, index) => (
              <li key={index} onClick={() => setSelected(index)} className={(index===selected) ? "revenueTableEntry revenueTableEntrySelected" : "revenueTableEntry"}>
                <p><strong>{"TreeIndex: "}{entry[0]}</strong></p>
                <p><i>Title: </i><strong>{metaList[index].title}</strong></p>
                <p><i>Contract: </i>{metaList[index].contract}</p>
                <p><i>Root ID: </i>{metaList[index].rootId}</p>
                <p><i>{"Price: "}</i>{utils.format.formatNearAmount(metaList[index].price)}</p>
                <p><i>Revenue Table:</i></p>
                <ul>
                {Object.keys(entry[1].revenue_table).map((line, ix) => {
                  return <li key={ix}>{line}{" : "}{entry[1].revenue_table[line]/100}{" %"}</li>
                })}
                </ul>
              </li>
            ))}
          </ul>
        </section>}

        <section>
          <p>{"Fill in the new values:"}</p>
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
          <button onClick={initiateAlterRevenue}>{"Alter Revenue Table"}</button>
        </section>

        <section>
          {readyForVote && <p className="finalizeMessage">You can click on finalize now!</p>}
          <p>{"We will need to act on the proposal that we've just created. See Registration"}</p>
          <p>{"Last proposal ID: "} {localStorage.getItem("last_proposal_id")}</p>
          <p>{"The ID of the proposal that we want to act on should be: "}<code>{proposalId}</code></p>
          <button onClick={finaliizeAlterRevenue}>{"Finalize Create Revenue Table"}</button>
          <p><code>{message}</code></p>
        </section>
      </main>
    </>
  )
}
