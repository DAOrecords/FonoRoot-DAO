import { utils } from 'near-api-js';
import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { actOnProposal, getLastProposalId, getListOfIncomeTables, getNftMetadata, getSingleIncomeTable, payout } from '../utils';


export default function Payout() {
  const [incomeTables, setIncomeTables] = useState([]);
  const [metaList, setMetaList] = useState([]);
  const [selectedSongs, setSelectedSongs] = useState([]);
  const [readyForVote, setReadyForVote] = useState(false);
  const [message, setMessage] = useState("");
  const [proposalId, setProposalId] = useState(null);

  // Get the list of IncomeTables
  useEffect(async () => {
    const list = await getListOfIncomeTables(0, 1000);
    setIncomeTables(list);
  }, []);

  async function initiatePayout() {
    if (selectedSongs.length === 0)  { console.log("No entry is selected."); return; }

    const lastProposalId = await getLastProposalId();
    localStorage.setItem("last_proposal_id", lastProposalId);

    const returnedProposalID = await payout(selectedSongs);
    localStorage.setItem("last_proposal_id", returnedProposalID);
    setProposalId(returnedProposalID);
    setReadyForVote(true);
  }

  // Act on proposal for Payout
  async function finaliizePayout() {
    console.log(proposalId)
    const returnMessage = await actOnProposal(proposalId);
    setMessage(JSON.stringify(returnMessage));
  }

  function addSong(treeIndex) {
    let newArray = selectedSongs;
    newArray.push(treeIndex);
    newArray = newArray.sort();
    newArray = [... new Set(newArray)];
    setSelectedSongs(Object.assign([], newArray));
  }

  function removeSong(treeIndex) {
    let newArray = selectedSongs;
    const position = selectedSongs.findIndex((element) => element === treeIndex);
    selectedSongs.splice(position, 1);
    setSelectedSongs(Object.assign([], newArray));
  }


  return (
    <>
      <Nav />
      <main>
        <h2>Payout</h2>

        <section>
          {"Select songs that you want to pay out"}
          <ul className="revenueList">
            {incomeTables.map((IncomeTable) => (
              <li 
                className={(selectedSongs.includes(IncomeTable[0])) ? "incomeTableEntry incomeTableEntrySelected" : "incomeTableEntry"} 
                onClick={(selectedSongs.includes(IncomeTable[0])) ? () => removeSong(IncomeTable[0]) : () => addSong(IncomeTable[0])} 
                key={IncomeTable[0]}
              >
                <p><strong>{"TreeIndex: "}{IncomeTable[0]}</strong></p>
                <p><i>{"Owner: "}</i>{IncomeTable[1].owner}</p>
                <p><i>{"Current Balance: "}</i>
                  {(IncomeTable[1].current_balance === 0) ? "0" : utils.format.formatNearAmount(BigInt(IncomeTable[1].current_balance), 2)}{" NEAR"}
                </p>
                <p><i>{"Total Income: "}</i>
                  {(IncomeTable[1].total_income === 0) ? "0" : utils.format.formatNearAmount(BigInt(IncomeTable[1].total_income), 2)}{" NEAR"}
                </p>
              </li>
            ))}
          </ul>
        </section>

        <section>
          <p>{"A song can be paid out if"}</p>
          <p>{"1. the song is owned by the logged in user"}</p>
          <p>{"2. the logged in user is a council member"}</p>
          <p>{"The smart contract will pay out the songs that can be paid out according to the above rules, the remaining songs will not be paid out."}</p>
          <p>{"Payout is happening according to the RevenueTable"}</p>
          
          <button onClick={initiatePayout}>Initiate Payout</button>
        </section>

        <section>
          {readyForVote && <p className="finalizeMessage">You can click on finalize now!</p>}
          <p>{"We will need to act on the proposal that we've just created. See Registration"}</p>
          <p>{"Last proposal ID: "} {localStorage.getItem("last_proposal_id")}</p>
          <p>{"The ID of the proposal that we want to act on should be: "}<code>{proposalId}</code></p>
          <button onClick={finaliizePayout}>{"Finalize Payout"}</button>
          <p><code>{message}</code></p>
        </section>
      </main>
    </>
  )
}
