<?php

/**
 *
 * nqrustbackup-webui - NQRust Backup Web Console
 *
 * @link      https://github.com/nqrustbackup/nqrustbackup for the canonical source repository
 * @copyright Copyright (C) 2013-2025 NQRustBackup GmbH & Co. KG (http://www.nqrustbackup.org/)
 * @license   GNU Affero General Public License (http://www.gnu.org/licenses/)
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

namespace Api\Controller;

use Laminas\Mvc\Controller\AbstractRestfulController;
use Laminas\View\Model\JsonModel;
use Exception;

class ClientController extends AbstractRestfulController
{
    protected $bsock = null;
    protected $clientModel = null;
    protected $result = null;

    public function getList()
    {
        $this->RequestURIPlugin()->setRequestURI();

        if (!$this->SessionTimeoutPlugin()->isValid()) {
            return $this->redirect()->toRoute(
                'auth',
                array(
                    'action' => 'login'
                ),
                array(
                    'query' => array(
                        'req' => $this->RequestURIPlugin()->getRequestURI(),
                        'dird' => $_SESSION['nqrustbackup']['director']
                    )
                )
            );
        }
        $this->bsock = $this->getServiceLocator()->get('director');
        $client = $this->params()->fromQuery('client');

        try {
            if (isset($client)) {
                $this->result = $this->getClientModel()->getClient($this->bsock, $client);
            } else {
                # $_SESSION['nqrustbackup']['product-updates'] is
                # a sorted list of NQRustBackup release versions.
                $nqrustbackup_supported_versions = $_SESSION['nqrustbackup']['product-updates'];

                $clients = $this->getClientModel()->getClients($this->bsock);
                $dot_clients = $this->getClientModel()->getDotClients($this->bsock);

                $this->result = array();

                for ($i = 0; $i < count($clients); $i++) {
                    $this->result[$i]['clientid'] = $clients[$i]['clientid'];
                    $this->result[$i]['uname'] = $clients[$i]['uname'];
                    $this->result[$i]['name'] = $clients[$i]['name'];
                    $this->result[$i]['autoprune'] = $clients[$i]['autoprune'];
                    $this->result[$i]['fileretention'] = $clients[$i]['fileretention'];
                    $this->result[$i]['jobretention'] = $clients[$i]['jobretention'];
                    $uname = explode(",", $clients[$i]['uname']);
                    $v = explode(" ", $uname[0]);
                    $this->result[$i]['version'] = $v[0];
                    $this->result[$i]['version_tooltip'] = "";
                    $this->result[$i]['version_status'] = "unknown";
                    $version_info = \NQRustBackup\Util::getNearestVersionInfo($nqrustbackup_supported_versions, $this->result[$i]['version']);
                    if ($version_info) {
                        $this->result[$i]['version_tooltip'] = $version_info['package_update_info'];
                        $this->result[$i]['version_status'] = $version_info['status'];
                    }
                    $this->result[$i]['enabled'] = "";
                    for ($j = 0; $j < count($dot_clients); $j++) {
                        if ($this->result[$i]['name'] == $dot_clients[$j]['name']) {
                            $this->result[$i]['enabled'] = $dot_clients[$j]['enabled'];
                        }
                    }
                }
            }
        } catch(Exception $e) {
            $this->getResponse()->setStatusCode(500);
            error_log($e);
        }

        return new JsonModel($this->result);
    }

    public function getClientModel()
    {
        if (!$this->clientModel) {
            $sm = $this->getServiceLocator();
            $this->clientModel = $sm->get('Client\Model\ClientModel');
        }
        return $this->clientModel;
    }
}
